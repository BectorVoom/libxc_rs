//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1572/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1572(t10785: f64, t10786: f64, t2747: f64, t10730: f64, t10734: f64, t10737: f64, t10742: f64, t10746: f64, t10749: f64, t10752: f64, t10756: f64, t10758: f64, t10762: f64, t10766: f64, t10773: f64, t10783: f64, t2730: f64, t2745: f64, t4362: f64, t851: f64) -> (f64, f64) {
    let t10788 = t2747 * t10785 * t10786;
    let t10791 = -0.42874018118069736972e-4_f64 * t10730 + 0.21437009059034868486e-4_f64 * t10734 - 0.85748036236139473944e-3_f64 * t851 * t10737 - 0.15246000842785598468e-3_f64 * t10742 + 0.76230004213927992336e-5_f64 * t10746 - 0.5421477899694558815e-4_f64 * t10749 + 3.0_f64 / 16.0_f64 * t2730 * t10752 - t10756 - t10758 - 0.13553694749236397037e-4_f64 * t10762 + 0.25724410870841842183e-2_f64 * t2745 * t10766 - 0.12862205435420921092e-1_f64 * t2745 * t10773 + 0.30492001685571196935e-3_f64 * t10783 - 0.51448821741683684367e-2_f64 * t4362 * t10788;
    (t10788, t10791)
}
