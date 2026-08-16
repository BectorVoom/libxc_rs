//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1330/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1330(t26739: f64, t27856: f64, t1020: f64, t26706: f64, t27836: f64, t26710: f64, t2842: f64, t1250: f64, t2175: f64, t27964: f64, t44454: f64, t7711: f64, t8034: f64, t93620: f64, t93759: f64, t93762: f64, t93764: f64, t93767: f64, t93771: f64, t93773: f64) -> (f64, f64, f64) {
    let t96456 = 0.16489724537037037037e-3_f64 * t26739 * t27856;
    let t96469 = t1020 * t27836 * t26706;
    let t96472 = t2842 * t27836 * t26710;
    let t96474 = -0.37069444444444444444e-2_f64 * t27964 * t7711 - t96456 - 0.30891203703703703704e-3_f64 * t93759 - 0.30891203703703703704e-3_f64 * t93762 + 0.46336805555555555556e-3_f64 * t93764 + 0.23168402777777777778e-3_f64 * t93767 + 0.6183646701388888889e-4_f64 * t93771 + 0.30918233506944444445e-4_f64 * t93773 - 0.69505208333333333333e-3_f64 * t44454 * t1250 * t2175 + 0.90693484953703703702e-3_f64 * t93620 * t8034 + 0.16581944444444444444e-2_f64 * t96469 + 0.27636574074074074073e-2_f64 * t96472;
    (t96469, t96472, t96474)
}
