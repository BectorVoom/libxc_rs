//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta597 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2240;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2241;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2242;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta597<F: Float>(t23920: F, t3092: F, t1011: F, t1063: F, t11737: F, t15618: F, t15712: F, t15732: F, t15750: F, t19786: F, t19827: F, t19867: F, t19883: F, t23874: F, t23878: F, t23886: F, t23892: F, t23900: F, t23904: F, t23908: F, t23913: F, t23917: F, t3091: F, t3127: F, t4834: F, t4892: F, t4899: F, t6268: F, t6331: F, t1668: F, t3154: F, t19572: F, t3117: F, t357: F, t15696: F, t6267: F, t23503: F, t4915: F, t11890: F, t15189: F, t18919: F, t18924: F, t18934: F, t23479: F, t23483: F, t23487: F, t23490: F, t23501: F, t23505: F, t341: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t23921, t23926) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2240::<F>(t23920, t3092, t1011, t1063, t11737, t15618, t15712, t15732, t15750, t19786, t19827, t19867, t19883, t23874, t23878, t23886, t23892, t23900, t23904, t23908, t23913, t23917, t3091, t3127, t4834, t4892, t4899, t6268, t6331);
        let (t23929, t23930, t23931, t23934, t23935, t23936, t23939, t23945, t23958) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2241::<F>(t1668, t3154, t19572, t3117, t357, t15696, t6267, t23503, t4915, t11890, t15189, t18919, t18924, t18934, t23479, t23483, t23487, t23490, t23501, t23505);
        let t23959 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2242::<F>(t23958, t341);
    (t23921, t23926, t23929, t23930, t23931, t23934, t23935, t23936, t23939, t23945, t23958, t23959)
}
