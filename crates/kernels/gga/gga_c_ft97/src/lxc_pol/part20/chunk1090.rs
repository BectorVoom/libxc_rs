//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1090/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1090<F: Float>(t24298: F, t27506: F, t172: F, t27505: F, t6043: F, t6046: F, t24378: F, t27665: F, t6034: F, t14842: F, t173: F, t27670: F, t27671: F, t1127: F, t13407: F, t2378: F, t2393: F, t2395: F, t2405: F, t2409: F, t2413: F, t2418: F, t24276: F, t24324: F, t24325: F, t24361: F, t24367: F, t24372: F, t24374: F, t2440: F, t27537: F, t27538: F, t27584: F, t27637: F, t27642: F, t27669: F, t27673: F, t3774: F, t6035: F, t6055: F, t65698: F, t96600: F, t992: F) -> (F, F, F) {
    let t108679 = t27506 * t24298;
    let t108685 = t27505 * t172;
    let t108688 = 0.6809984893827160494e-1 * t6043 * t108685 * t6046;
    let t108697 = 0.14846767889314528222e-3 * t6034 * t24378 * t27665;
    let t108733 = t27670 * t27671 * t173 * t14842;
    let t108735 = 0.2269994964609053498e-1 * t6055 * t108679 - 0.13784064983740990796e-3 * t3774 * t27584 * t13407 - t108688 + 0.30644932022222222223e0 * t24324 * t27506 * t24325 + 0.31073410497668637766e-5 * t65698 * t27669 * t27673 + t108697 + 0.74233839446572641111e-4 * t24276 * t27537 * t27538 * t2418 - 0.4945510644553639738e-5 * t96600 * t27537 * t992 * t2378 * t2395 - 0.14846767889314528222e-4 * t24276 * t27537 * t992 * t2393 * t2395 + 0.12768721675925925926e-1 * t24361 * t6035 * t27637 * t2413 + 0.17024962234567901235e-1 * t24361 * t6035 * t2440 * t1127 * t2405 - 0.59387071557258112888e-3 * t6034 * t27642 * t24367 + 0.39564085156429117903e-4 * t24372 * t27642 * t24374 - 0.25537443351851851852e-1 * t24361 * t6035 * t27637 * t2409 - 0.34526011664076264184e-5 * t108733;
    (t108679, t108685, t108735)
}
