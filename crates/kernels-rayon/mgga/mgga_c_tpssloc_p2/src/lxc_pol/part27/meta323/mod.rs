//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta323 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1397;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1398;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta323(t3247: f64, t460: f64, t2244: f64, t1176: f64, t134: f64, t1184: f64, t3451: f64, t3447: f64, t3448: f64, t3475: f64, t1239: f64, t68: f64, t225: f64, t3484: f64, t1222: f64, t3567: f64, t1203: f64, t3540: f64, t2393: f64, t374: f64, t486: f64, t485: f64, t248: f64, t3516: f64, t3570: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11583, t11584, t11588, t11589, t11591, t11593, t11606) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1397(t3247, t460, t2244, t1176, t134, t1184, t3451, t3447, t3448, t3475, t1239, t68);
        let (t11613, t11642, t11644, t11649, t11651) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1398(t225, t3484, t1222, t3567, t1203, t3540, t2393, t374, t486, t485, t248, t3516, t3570);
    (t11583, t11584, t11588, t11589, t11591, t11593, t11606, t11613, t11642, t11644, t11649, t11651)
}
