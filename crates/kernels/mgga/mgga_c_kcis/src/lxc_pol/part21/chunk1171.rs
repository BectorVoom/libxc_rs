//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1171/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1171<F: Float>(t26739: F, t27856: F, t1020: F, t26706: F, t27836: F, t26710: F, t2842: F, t1250: F, t2175: F, t27964: F, t44454: F, t7711: F, t8034: F, t93620: F, t93759: F, t93762: F, t93764: F, t93767: F, t93771: F, t93773: F) -> (F, F, F) {
    let t96456 = 0.16489724537037037037e-3 * t26739 * t27856;
    let t96469 = t1020 * t27836 * t26706;
    let t96472 = t2842 * t27836 * t26710;
    let t96474 = -0.37069444444444444444e-2 * t27964 * t7711 - t96456 - 0.30891203703703703704e-3 * t93759 - 0.30891203703703703704e-3 * t93762 + 0.46336805555555555556e-3 * t93764 + 0.23168402777777777778e-3 * t93767 + 0.6183646701388888889e-4 * t93771 + 0.30918233506944444445e-4 * t93773 - 0.69505208333333333333e-3 * t44454 * t1250 * t2175 + 0.90693484953703703702e-3 * t93620 * t8034 + 0.16581944444444444444e-2 * t96469 + 0.27636574074074074073e-2 * t96472;
    (t96469, t96472, t96474)
}
