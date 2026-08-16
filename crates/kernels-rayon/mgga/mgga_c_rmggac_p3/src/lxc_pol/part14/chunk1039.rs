//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1039/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1039(t1685: f64, t2127: f64, t36505: f64, t36506: f64, t36508: f64, t41605: f64, t41607: f64, t41610: f64, t41614: f64, t41616: f64, t41620: f64, t41627: f64, t41631: f64, t41635: f64, t41637: f64, t41639: f64, t41641: f64, t6473: f64, t72: f64, t7772: f64) -> f64 {
    let t41645 = -t41605 - 0.72042316457491791906e-3_f64 * t41607 - 0.72042316457491791906e-3_f64 * t41610 - t41614 - 0.72042316457491791906e-3_f64 * t41616 - t41620 + 2.0_f64 * t72 * t1685 * t2127 + 0.1064114997332445985e-4_f64 * t41627 - 0.23948483403727617128e0_f64 * t6473 * t7772 - 0.27274661654245341728e-1_f64 * t41631 - 0.40911992481368012592e0_f64 * t41635 - 0.36366215538993788971e0_f64 * t41637 - 0.81823984962736025184e-1_f64 * t41639 + 0.21819729323396273382e0_f64 * t41641 + t36505 + 0.99317399751028291929e-5_f64 * t36506 - 0.66211599834018861286e-4_f64 * t36508;
    t41645
}
