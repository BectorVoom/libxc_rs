//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta365 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1498;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1499;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1500;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1501;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1502;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta365(t13520: f64, t2845: f64, t10650: f64, t1557: f64, t2787: f64, t4396: f64, t2770: f64, t3966: f64, t607: f64, t2826: f64, t136: f64, t2250: f64, t4337: f64, t10216: f64, t1409: f64, t2244: f64, t10304: f64, t2775: f64, t908: f64, t4342: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t13522, t13524, t13526, t13528) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1498(t13520, t2845, t10650, t1557, t2787, t4396, t2770, t3966, t607);
        let (t13530, t13532) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1499(t13528, t2826, t136, t2250, t4337);
        let (t13534, t13537) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1500(t13532, t2826, t136, t10216, t1409, t2244);
        let (t13539, t13542) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1501(t10304, t13537, t136, t2775, t3966, t607);
        let (t13544, t13546) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1502(t13542, t908, t136, t2250, t4342);
    (t13522, t13524, t13526, t13528, t13530, t13532, t13534, t13537, t13539, t13542, t13544, t13546)
}
