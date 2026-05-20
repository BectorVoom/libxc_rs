//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta584 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2211;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2212;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2213;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta584<F: Float>(t4578: F, t5825: F, t904: F, t128: F, t23499: F, t2908: F, t141: F, t930: F, t15123: F, t15189: F, t23472: F, t23476: F, t23479: F, t23483: F, t23487: F, t23490: F, t23493: F, t23496: F, t23501: F, t4598: F, t6120: F, t4614: F, t11304: F, t18919: F, t18924: F, t18934: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t23503, t23504, t23505) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2211::<F>(t4578, t5825, t904, t128);
        let (t23507, t23508, t23510, t23511, t23514) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2212::<F>(t23499, t2908, t141, t23503, t930, t15123, t15189, t23472, t23476, t23479, t23483, t23487, t23490, t23493, t23496, t23501, t23505);
        let (t23521, t23523, t23535) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2213::<F>(t4598, t6120, t4614, t11304, t15189, t18919, t18924, t18934, t23479, t23483, t23487, t23490, t23501, t23505);
    (t23503, t23504, t23505, t23507, t23508, t23510, t23511, t23514, t23521, t23523, t23535)
}
