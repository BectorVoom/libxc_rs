//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta346 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1203;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1204;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1205;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta346<F: Float>(t23705: F, t2970: F, t15123: F, t15189: F, t23472: F, t23476: F, t23479: F, t23483: F, t23487: F, t23490: F, t23493: F, t23496: F, t23501: F, t23505: F, t23508: F, t23511: F, t11422: F, t11423: F, t18919: F, t18924: F, t18934: F, t19002: F, t19004: F, t19009: F, t23521: F, t23523: F, t23536: F, t23538: F, t23541: F, t23543: F, t954: F, t1621: F, t19275: F, t1634: F, t6205: F, t1633: F, t19303: F, t1610: F, t6141: F, t2874: F) -> (F, F, F, F, F, F, F, F) {
        let (t23723, t23740) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1203::<F>(t23705, t2970, t15123, t15189, t23472, t23476, t23479, t23483, t23487, t23490, t23493, t23496, t23501, t23505, t23508, t23511);
        let t23753 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1204::<F>(t11422, t11423, t18919, t18924, t18934, t19002, t19004, t19009, t23521, t23523, t23536, t23538, t23541, t23543);
        let (t23754, t23755, t23758, t23761, t23764, t23767, t23769) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1205::<F>(t23740, t23753, t954, t1621, t19275, t1634, t6205, t1633, t19303, t1610, t6141, t2874);
    (t23723, t23754, t23755, t23758, t23761, t23764, t23767, t23769)
}
