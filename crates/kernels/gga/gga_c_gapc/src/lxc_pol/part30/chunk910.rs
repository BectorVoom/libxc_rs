//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 910/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk910<F: Float>(t3563: F, t883: F, t1117: F, t2468: F, t10103: F, t10106: F, t10108: F, t10111: F, t10115: F, t10118: F, t10120: F, t10126: F, t10128: F, t10131: F, t10134: F) -> (F, F, F) {
    let t11043 = t3563 * t883;
    let t11046 = t1117 * t2468;
    let t11060 = -F::cast_from(0.54715885245250729722e-5_f64) * t10103 + F::cast_from(0.26446011201871186032e-4_f64) * t10106 + F::cast_from(0.25051693218177510181e-2_f64) * t10108 + F::cast_from(0.23485962392041415794e-5_f64) * t10111 + F::cast_from(0.3757753982726626527e-4_f64) * t10115 + F::cast_from(0.54715885245250729722e-5_f64) * t10118 + F::cast_from(0.18968173551686919637e-3_f64) * t10120 + F::cast_from(0.39597758471766536049e-5_f64) * t10126 + F::cast_from(0.29230628793134746097e-4_f64) * t10128 - F::cast_from(0.56366309740899397906e-3_f64) * t10131 - F::cast_from(0.3757753982726626527e-4_f64) * t10134;
    (t11043, t11046, t11060)
}
