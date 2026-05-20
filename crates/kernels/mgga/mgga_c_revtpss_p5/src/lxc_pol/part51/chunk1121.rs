//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1121/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1121<F: Float>(t125830: F, t32710: F, t5659: F, t7301: F, t5710: F, t8477: F, t32272: F, t33970: F, t121031: F, t121076: F, t121077: F, t121212: F, t121214: F, t121219: F, t121228: F, t121230: F, t121233: F, t121235: F, t125819: F, t125821: F, t125826: F, t125831: F, t27845: F, t27846: F, t27864: F, t27903: F, t27972: F, t27980: F, t32233: F, t32719: F, t8586: F) -> (F, F) {
    let t125833 = t32710 * t125830;
    let t125835 = t7301 * t5659;
    let t125849 = t8477 * t5710;
    let t125855 = t32272 * t33970;
    let t125857 = F::cast_from(0.56468933516960933998e-3_f64) * t125819 - F::cast_from(0.52041769129231196772e1_f64) * t32233 * t125821 + F::cast_from(0.17347256376410398924e1_f64) * t121031 * t27846 - F::cast_from(0.13386901839087538754e-3_f64) * t125826 - F::cast_from(0.3718732920905101082e-4_f64) * t125831 + F::cast_from(0.66119071333692697238e-4_f64) * t125833 + F::cast_from(0.8673628188205199462e0_f64) * t32233 * t125835 + F::cast_from(0.34694512752820797848e1_f64) * t121031 * t27903 - t121212 - t121214 + F::cast_from(0.3427184259906141157e1_f64) * t121076 * t121077 * t27972 - F::cast_from(0.22847895066040941046e1_f64) * t32719 * t27980 * t27845 + F::cast_from(0.34271842599061411569e1_f64) * t121076 * t121077 * t27864 - t121219 + F::cast_from(0.57119737665102352616e0_f64) * t125849 * t8586 - t121228 - F::cast_from(0.14279934416275588154e-1_f64) * t121230 + F::cast_from(0.75291911355947911996e-4_f64) * t121233 + F::cast_from(0.18822977838986977999e-4_f64) * t121235 + F::cast_from(0.7437465841810202164e-3_f64) * t125855;
    (t125849, t125857)
}
