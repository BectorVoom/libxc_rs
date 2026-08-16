//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1071/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1071(t11436: f64, t366: f64, t2703: f64, t2785: f64, t345: f64, t2723: f64, t9080: f64, t1474: f64, t11707: f64, t11733: f64, t11743: f64, t11750: f64, t11753: f64, t1477: f64, t220: f64, t2782: f64, t2786: f64, t2798: f64, t2799: f64, t368: f64, t3987: f64, t3997: f64, t4001: f64, t4004: f64, t4008: f64, t9077: f64, t9089: f64, t9094: f64, t9117: f64, t948: f64, t983: f64, t985: f64) -> f64 {
    let t11760 = t366 * t11436;
    let t11767 = t2785 * t2703 * t345;
    let t11771 = t9080 * t2723 * t345;
    let t11774 = t1474 * t2723;
    let t11782 = t1474 * t2703;
    let t11789 = 2.0_f64 * t3987 * t948 * t983 * t985 + t11707 * t220 * t368 + 6.0_f64 * t11733 * t1477 * t9077 - 6.0_f64 * t11743 * t1477 * t9094 + t11750 * t983 * t985 + 2.0_f64 * t11753 * t983 * t985 + t11760 * t983 * t985 - t11767 * t1477 * t2798 + t11771 * t1477 * t9117 + 2.0_f64 * t11774 * t2782 * t2786 - t11774 * t2798 * t2799 + t11782 * t983 * t985 + 2.0_f64 * t1477 * t2782 * t9089 + 4.0_f64 * t2782 * t3997 * t4001 + 4.0_f64 * t2782 * t3997 * t4004 - 2.0_f64 * t2798 * t4001 * t4008 - 2.0_f64 * t2798 * t4004 * t4008;
    t11789
}
