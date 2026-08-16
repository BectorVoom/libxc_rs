//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 797/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk797(t12535: f64, t12549: f64, t550: f64, t133: f64, t132: f64, t2030: f64, t1009: f64, t2037: f64, t1010: f64, t1014: f64, t12418: f64, t12422: f64, t12425: f64, t12435: f64, t12438: f64, t12441: f64, t12445: f64, t12449: f64, t12452: f64, t12455: f64, t12514: f64, t12516: f64, t12522: f64, t1683: f64, t2001: f64, t2036: f64, t2071: f64, t3350: f64, t3392: f64, t3393: f64, t538: f64, t554: f64, t8812: f64, t8885: f64) -> f64 {
    let t12550 = t12535 + t12549;
    let t12551 = t550 * t12550;
    let t12552 = t133 * t12551;
    let t12553 = t2030 * t132;
    let t12554 = t12553 * t1009;
    let t12556 = t2037 * t1009;
    let t12559 = -4.0_f64 * t2001 * t12418 * t538 - 2.0_f64 * t2001 * t12422 + 4.0_f64 * t3392 * t12425 * t554 + 2.0_f64 * t3392 * t3393 * t2071 - 0.72985269132393279984e0_f64 * t2036 * t2037 * t1014 + 0.2416365355361531912e1_f64 * t12435 * t12438 - 0.2416365355361531912e1_f64 * t12441 * t12438 - 0.2416365355361531912e1_f64 * t12445 * t8885 + 0.2416365355361531912e1_f64 * t12449 * t8885 - 0.2416365355361531912e1_f64 * t12452 * t8885 + 0.2416365355361531912e1_f64 * t12455 * t8885 + 2.0_f64 * t12514 - 2.0_f64 * t12516 - 0.76518236253115177213e1_f64 * t1010 * t1683 + 0.76518236253115177213e1_f64 * t3350 * t1683 + 4.0_f64 * t12522 - t12552 + 2.0_f64 * t12554 + 0.14597053826478655997e1_f64 * t8812 * t12556;
    t12559
}
