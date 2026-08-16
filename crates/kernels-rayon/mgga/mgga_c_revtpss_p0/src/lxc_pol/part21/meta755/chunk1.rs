//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2648/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2648(t13805: f64, t13847: f64, t13848: f64, t48731: f64, t1353: f64, t13789: f64, t13790: f64, t13804: f64, t13944: f64, t3924: f64, t3934: f64, t3936: f64, t4056: f64, t46800: f64, t46804: f64, t46810: f64, t47248: f64, t48595: f64, t48686: f64, t48687: f64, t48691: f64, t48692: f64, t48696: f64, t48700: f64, t48709: f64, t48712: f64, t543: f64, t5671: f64, t5673: f64, t5674: f64, t5675: f64, t5704: f64, t9628: f64, t9840: f64, t9984: f64) -> f64 {
    let t48734 = t48731 * t13847 * t13848 * t13805;
    let t48745 = t48686 - 7.0_f64 / 16.0_f64 * t48687 - t48691 - 0.91464571985215438874e-3_f64 * t48692 + 0.54214778996945588151e-4_f64 * t48696 + 0.5421477899694558815e-4_f64 * t48700 + t46800 + 0.85748036236139473944e-3_f64 * t3934 * t3936 * t5674 * t543 * t9628 + 0.15246000842785598468e-3_f64 * t48709 - 0.77173232612525526549e-1_f64 * t48712 * t47248 * t5704 * t9984 - 0.51448821741683684367e-2_f64 * t5671 * t13789 * t13790 * t1353 * t4056 - 0.64311027177104605458e-3_f64 * t3934 * t5673 * t13944 * t3924 - 0.38586616306262763275e-2_f64 * t13804 * t5673 * t13944 * t13805 - 0.22869001264178397702e-3_f64 * t48734 + 0.12862205435420921092e-2_f64 * t5671 * t5673 * t48595 * t5675 + 0.12862205435420921092e-2_f64 * t5671 * t5673 * t13944 * t9840 + 0.27107389498472794074e-4_f64 * t46804 + t46810;
    t48745
}
