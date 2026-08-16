//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2648/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2648<F: Float>(t13805: F, t13847: F, t13848: F, t48731: F, t1353: F, t13789: F, t13790: F, t13804: F, t13944: F, t3924: F, t3934: F, t3936: F, t4056: F, t46800: F, t46804: F, t46810: F, t47248: F, t48595: F, t48686: F, t48687: F, t48691: F, t48692: F, t48696: F, t48700: F, t48709: F, t48712: F, t543: F, t5671: F, t5673: F, t5674: F, t5675: F, t5704: F, t9628: F, t9840: F, t9984: F) -> F {
    let t48734 = t48731 * t13847 * t13848 * t13805;
    let t48745 = t48686 - F::cast_from(7.0_f64) / F::cast_from(16.0_f64) * t48687 - t48691 - F::cast_from(0.91464571985215438874e-3_f64) * t48692 + F::cast_from(0.54214778996945588151e-4_f64) * t48696 + F::cast_from(0.5421477899694558815e-4_f64) * t48700 + t46800 + F::cast_from(0.85748036236139473944e-3_f64) * t3934 * t3936 * t5674 * t543 * t9628 + F::cast_from(0.15246000842785598468e-3_f64) * t48709 - F::cast_from(0.77173232612525526549e-1_f64) * t48712 * t47248 * t5704 * t9984 - F::cast_from(0.51448821741683684367e-2_f64) * t5671 * t13789 * t13790 * t1353 * t4056 - F::cast_from(0.64311027177104605458e-3_f64) * t3934 * t5673 * t13944 * t3924 - F::cast_from(0.38586616306262763275e-2_f64) * t13804 * t5673 * t13944 * t13805 - F::cast_from(0.22869001264178397702e-3_f64) * t48734 + F::cast_from(0.12862205435420921092e-2_f64) * t5671 * t5673 * t48595 * t5675 + F::cast_from(0.12862205435420921092e-2_f64) * t5671 * t5673 * t13944 * t9840 + F::cast_from(0.27107389498472794074e-4_f64) * t46804 + t46810;
    t48745
}
