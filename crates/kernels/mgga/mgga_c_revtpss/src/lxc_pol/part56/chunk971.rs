//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 971/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk971<F: Float>(t32272: F, t33970: F, t121031: F, t121076: F, t121077: F, t121212: F, t121214: F, t121219: F, t121228: F, t121230: F, t121233: F, t121235: F, t125819: F, t125821: F, t125826: F, t125831: F, t125833: F, t125835: F, t125849: F, t27845: F, t27846: F, t27864: F, t27903: F, t27972: F, t27980: F, t32233: F, t32719: F, t8586: F) -> (F,) {
    let t125855 = t32272 * t33970;
    let t125857 = 0.56468933516960933998e-3 * t125819 - 0.52041769129231196772e1 * t32233 * t125821 + 0.17347256376410398924e1 * t121031 * t27846 - 0.13386901839087538754e-3 * t125826 - 0.3718732920905101082e-4 * t125831 + 0.66119071333692697238e-4 * t125833 + 0.8673628188205199462e0 * t32233 * t125835 + 0.34694512752820797848e1 * t121031 * t27903 - t121212 - t121214 + 0.3427184259906141157e1 * t121076 * t121077 * t27972 - 0.22847895066040941046e1 * t32719 * t27980 * t27845 + 0.34271842599061411569e1 * t121076 * t121077 * t27864 - t121219 + 0.57119737665102352616e0 * t125849 * t8586 - t121228 - 0.14279934416275588154e-1 * t121230 + 0.75291911355947911996e-4 * t121233 + 0.18822977838986977999e-4 * t121235 + 0.7437465841810202164e-3 * t125855;
    (t125857,)
}
