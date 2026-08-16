//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta402 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1697;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1698;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1699;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta402(t18416: f64, t3451: f64, t3448: f64, t6144: f64, t18225: f64, t4908: f64, t11583: f64, t5392: f64, t3449: f64, t18221: f64, t15320: f64, t4904: f64, t15313: f64, t4919: f64, t11531: f64, t15265: f64, t15376: f64, t18404: f64, t18410: f64, t18413: f64, t3447: f64, t4901: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18417, t18420) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1697(t18416, t3451, t3448, t6144);
        let (t18421, t18424, t18427) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1698(t18420, t3451, t18225, t4908, t11583, t5392);
        let (t18428, t18431, t18434, t18437, t18442) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1699(t18427, t3449, t18221, t4908, t15320, t4904, t15313, t4919, t11531, t15265, t15376, t18404, t18410, t18413, t18417, t18421, t18424, t3447, t4901);
    (t18417, t18420, t18421, t18424, t18427, t18428, t18431, t18434, t18437, t18442)
}
