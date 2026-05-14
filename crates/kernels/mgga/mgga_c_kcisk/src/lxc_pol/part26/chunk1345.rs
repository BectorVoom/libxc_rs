//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1345/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1345<F: Float>(t34774: F, t9442: F, t1333: F, t34780: F, t20160: F, t34748: F, t9446: F, t118919: F, t26865: F, t415: F, t468: F, t110106: F, t114580: F, t114783: F, t114784: F, t114790: F, t118882: F, t27016: F, t2718: F, t32008: F, t32022: F, t34693: F, t9438: F, t9796: F) -> (F, F, F) {
    let t119652 = t34774 * t9442;
    let t119654 = t1333 * t34780;
    let t119659 = t9446 * t20160 * t34748;
    let t119665 = t9446 * t118919;
    let t119670 = t415 * t468 * t26865;
    let t119672 = 0.27777777777777777779e-1 * t27016 * t9438 * t2718 - 0.34722222222222222223e-2 * t119652 - 0.33163888888888888888e-2 * t119654 - 0.36848765432098765431e-3 * t110106 + t114783 + 0.46296296296296296297e-2 * t114784 - 0.69444444444444444447e-2 * t119659 + 0.8041666666666666667e-2 * t114580 * t9796 + 0.55555555555555555557e-1 * t32022 * t34693 + t114790 - 0.69444444444444444447e-2 * t119665 - 0.26805555555555555556e-2 * t32008 * t118882 - 0.44218518518518518517e-2 * t119670;
    (t119654, t119670, t119672)
}
