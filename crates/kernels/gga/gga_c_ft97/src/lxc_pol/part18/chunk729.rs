//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 729/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk729<F: Float>(t132: F, t2030: F, t1009: F, t2037: F, t1010: F, t1014: F, t12418: F, t12422: F, t12425: F, t12435: F, t12438: F, t12441: F, t12445: F, t12449: F, t12452: F, t12455: F, t12514: F, t12516: F, t12522: F, t12552: F, t1683: F, t2001: F, t2036: F, t2071: F, t3350: F, t3392: F, t3393: F, t538: F, t554: F, t8812: F, t8885: F) -> (F, F) {
    let t12553 = t2030 * t132;
    let t12554 = t12553 * t1009;
    let t12556 = t2037 * t1009;
    let t12559 = -4.0 * t2001 * t12418 * t538 - 2.0 * t2001 * t12422 + 4.0 * t3392 * t12425 * t554 + 2.0 * t3392 * t3393 * t2071 - 0.72985269132393279984e0 * t2036 * t2037 * t1014 + 0.2416365355361531912e1 * t12435 * t12438 - 0.2416365355361531912e1 * t12441 * t12438 - 0.2416365355361531912e1 * t12445 * t8885 + 0.2416365355361531912e1 * t12449 * t8885 - 0.2416365355361531912e1 * t12452 * t8885 + 0.2416365355361531912e1 * t12455 * t8885 + 2.0 * t12514 - 2.0 * t12516 - 0.76518236253115177213e1 * t1010 * t1683 + 0.76518236253115177213e1 * t3350 * t1683 + 4.0 * t12522 - t12552 + 2.0 * t12554 + 0.14597053826478655997e1 * t8812 * t12556;
    (t12553, t12559)
}
