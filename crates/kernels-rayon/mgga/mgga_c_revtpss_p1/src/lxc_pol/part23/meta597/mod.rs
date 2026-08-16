//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta597 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2240;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2241;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2242;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta597(t23920: f64, t3092: f64, t1011: f64, t1063: f64, t11737: f64, t15618: f64, t15712: f64, t15732: f64, t15750: f64, t19786: f64, t19827: f64, t19867: f64, t19883: f64, t23874: f64, t23878: f64, t23886: f64, t23892: f64, t23900: f64, t23904: f64, t23908: f64, t23913: f64, t23917: f64, t3091: f64, t3127: f64, t4834: f64, t4892: f64, t4899: f64, t6268: f64, t6331: f64, t1668: f64, t3154: f64, t19572: f64, t3117: f64, t357: f64, t15696: f64, t6267: f64, t23503: f64, t4915: f64, t11890: f64, t15189: f64, t18919: f64, t18924: f64, t18934: f64, t23479: f64, t23483: f64, t23487: f64, t23490: f64, t23501: f64, t23505: f64, t341: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t23921, t23926) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2240(t23920, t3092, t1011, t1063, t11737, t15618, t15712, t15732, t15750, t19786, t19827, t19867, t19883, t23874, t23878, t23886, t23892, t23900, t23904, t23908, t23913, t23917, t3091, t3127, t4834, t4892, t4899, t6268, t6331);
        let (t23929, t23930, t23931, t23934, t23935, t23936, t23939, t23945, t23958) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2241(t1668, t3154, t19572, t3117, t357, t15696, t6267, t23503, t4915, t11890, t15189, t18919, t18924, t18934, t23479, t23483, t23487, t23490, t23501, t23505);
        let t23959 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2242(t23958, t341);
    (t23921, t23926, t23929, t23930, t23931, t23934, t23935, t23936, t23939, t23945, t23958, t23959)
}
