//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 805/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk805<F: Float>(t3563: F, t883: F, t1117: F, t2468: F, t10103: F, t10106: F, t10108: F, t10111: F, t10115: F, t10118: F, t10120: F, t10126: F, t10128: F, t10131: F, t10134: F, t10137: F, t10140: F, t10144: F, t10148: F, t10151: F, t10154: F, t10156: F, t10160: F, t10163: F, t10165: F, t10168: F) -> (F, F, F, F) {
    let t11043 = t3563 * t883;
    let t11046 = t1117 * t2468;
    let t11060 = -0.54715885245250729722e-5 * t10103 + 0.26446011201871186032e-4 * t10106 + 0.25051693218177510181e-2 * t10108 + 0.23485962392041415794e-5 * t10111 + 0.3757753982726626527e-4 * t10115 + 0.54715885245250729722e-5 * t10118 + 0.18968173551686919637e-3 * t10120 + 0.39597758471766536049e-5 * t10126 + 0.29230628793134746097e-4 * t10128 - 0.56366309740899397906e-3 * t10131 - 0.3757753982726626527e-4 * t10134;
    let t11072 = -0.11273261948179879581e-2 * t10137 - 0.3757753982726626527e-4 * t10140 - 0.11273261948179879581e-2 * t10144 + 0.11273261948179879581e-2 * t10148 + 0.11273261948179879581e-2 * t10151 + 0.7113065081882594864e-4 * t10154 + 0.7113065081882594864e-4 * t10156 - 0.16414765573575218917e-4 * t10160 - 0.2227095527095980655e-5 * t10163 - 0.16440173674428991056e-4 * t10165 + 0.56366309740899397906e-3 * t10168;
    (t11043, t11046, t11060, t11072)
}
