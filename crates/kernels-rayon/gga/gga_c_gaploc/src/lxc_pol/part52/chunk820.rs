//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 820/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk820(t13327: f64, t6305: f64, t13313: f64, t6313: f64, t42717: f64, t42721: f64, t13304: f64, t484: f64, t2325: f64, t36117: f64, t882: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44487 = 0.28455006635676149599e-1_f64 * t6305 * t13327;
    let t44489 = 0.1138200265427045984e0_f64 * t6313 * t13313;
    let t44490 = 0.94850022118920498664e-2_f64 * t42717;
    let t44491 = 0.47425011059460249332e-2_f64 * t42721;
    let t44492 = t484 * t13304;
    let t44493 = 0.15808337019820083111e-2_f64 * t44492;
    let t44512 = t882 * t2325 * t883 * t36117;
    (t44487, t44489, t44490, t44491, t44493, t44512)
}
