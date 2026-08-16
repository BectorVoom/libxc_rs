//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 842/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk842(t36178: f64, t874: f64, t2268: f64, t2343: f64, t13268: f64, t6313: f64, t13327: f64, t6305: f64, t13313: f64, t42717: f64, t42721: f64, t13304: f64, t484: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t44480 = t36178 * t874;
    let t44483 = 0.56910013271352299198e-1_f64 * t2268 * t2343 * t44480;
    let t44485 = 0.45528010617081839357e0_f64 * t6313 * t13268;
    let t44487 = 0.28455006635676149599e-1_f64 * t6305 * t13327;
    let t44489 = 0.1138200265427045984e0_f64 * t6313 * t13313;
    let t44490 = 0.94850022118920498664e-2_f64 * t42717;
    let t44491 = 0.47425011059460249332e-2_f64 * t42721;
    let t44492 = t484 * t13304;
    (t44480, t44483, t44485, t44487, t44489, t44490, t44491, t44492)
}
