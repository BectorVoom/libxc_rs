//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 666/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk666(t12255: f64, t313: f64, t12223: f64, t701: f64, t1445: f64, t1457: f64, t10022: f64, t10026: f64, t10030: f64, t10042: f64, t11059: f64, t11063: f64, t11067: f64, t11071: f64, t11108: f64, t11111: f64, t11118: f64, t11121: f64, t12252: f64, t2004: f64, t2028: f64, t2639: f64, t807: f64) -> (f64, f64, f64) {
    let t12256 = t313 * t12255;
    let t12259 = t12223 * t701;
    let t12260 = t1445 * t12259;
    let t12263 = t1457 * t12259;
    let t12267 = -t11059 + t11063 - t11067 + t11071 + t11108 - t10022 - 0.39722766613167140743e-1_f64 * t12252 * t2028 - 0.10725146985555128001e1_f64 * t12256 * t2639 + 0.23005755572352449806e1_f64 * t807 * t12260 + 0.35750489951850426669e0_f64 * t2004 * t12263 - t10026 - 0.51123901271894332903e0_f64 * t10030 + t10042 - t11111 - t11118 + t11121;
    (t12256, t12259, t12267)
}
