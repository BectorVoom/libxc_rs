//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 942/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk942(t1931: f64, t1986: f64, t7720: f64, t1356: f64, t1632: f64, t2024: f64, t2402: f64, t289: f64, t39591: f64, t45614: f64, t45617: f64, t45622: f64, t45626: f64, t45630: f64, t45633: f64, t45636: f64, t45641: f64, t45646: f64, t45648: f64, t45651: f64, t45656: f64, t45660: f64, t45664: f64, t884: f64, t903: f64) -> f64 {
    let t45666 = t1986 * t1931;
    let t45667 = t7720 * t45666;
    let t45669 = 0.25538759935978703638e-4_f64 * t45614 + 0.25538759935978703638e-4_f64 * t45617 + 0.35922725105591425692e0_f64 * t903 * t2402 * t1632 - 0.11974241701863808564e0_f64 * t884 * t2024 * t45622 + 0.39914139006212695214e-1_f64 * t1356 * t45626 - 0.40650199722100037752e-3_f64 * t45630 - 0.81300399444200075504e-3_f64 * t45633 - 0.40650199722100037752e-3_f64 * t45636 + 0.1064114997332445985e-4_f64 * t45641 - 0.1064114997332445985e-4_f64 * t45646 - 0.25538759935978703638e-4_f64 * t45648 - 0.74488049813271218946e-4_f64 * t39591 - 0.2363e1_f64 * t289 * t45651 - 0.51077519871957407276e-4_f64 * t45656 + 0.76616279807936110914e-4_f64 * t45660 + 0.51077519871957407276e-4_f64 * t45664 - 0.25538759935978703638e-4_f64 * t45667;
    t45669
}
