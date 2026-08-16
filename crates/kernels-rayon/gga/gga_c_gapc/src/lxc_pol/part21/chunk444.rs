//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 444/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk444(t2415: f64, t297: f64, t19: f64, t314: f64, t20: f64, t1191: f64, t2262: f64, t2265: f64, t2269: f64, t2301: f64, t2305: f64, t2308: f64, t2311: f64, t2317: f64, t2406: f64, t2409: f64, t2414: f64, t771: f64, t788: f64, t794: f64, t795: f64, t835: f64, t877: f64) -> (f64, f64, f64, f64) {
    let t2416 = t2415 * t297;
    let t2417 = t314 * t19;
    let t2418 = t2417 * t20;
    let t2419 = t2416 * t2418;
    let t2422 = -t1191 - 0.56366309740899397906e-3_f64 * t2262 * t877 - 0.18788769913633132635e-4_f64 * t2265 * t795 - 0.56366309740899397906e-3_f64 * t771 * t2269 - 0.28183154870449698953e-3_f64 * t771 * t2301 + 0.93943849568165663176e-3_f64 * t2305 * t788 + 0.93943849568165663176e-3_f64 * t2308 * t877 + 0.43840463131810642816e-4_f64 * t2311 * t795 + 0.56366309740899397906e-3_f64 * t835 * t2317 - 0.28183154870449698953e-3_f64 * t835 * t2406 - 0.18788769913633132635e-4_f64 * t794 * t2409 + 0.19798879235883268025e-5_f64 * t2414 * t2419;
    (t2416, t2417, t2418, t2422)
}
