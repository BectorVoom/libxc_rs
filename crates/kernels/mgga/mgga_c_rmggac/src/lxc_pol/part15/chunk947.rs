//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 947/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk947<F: Float>(t2004: F, t47854: F, t2007: F, t1971: F, t2144: F, t30311: F, t3351: F, t36913: F, t36916: F, t36922: F, t36925: F, t36936: F, t36943: F, t36948: F, t42087: F, t42101: F, t47831: F, t47833: F, t47835: F, t47840: F, t47845: F) -> (F,) {
    let t47855 = t47854 * t2004;
    let t47857 = t47854 * t2007;
    let t47861 = t3351 * t1971 * t2144 * t30311;
    let t47863 = -0.5987120850931904282e-1 * t47831 - 0.71845450211182851384e0 * t47833 + 0.15965655602485078085e0 * t47835 + t42087 - 0.31923449919973379548e-4 * t47840 + 0.95770349759920138644e-4 * t47845 - 0.59590439850616975158e-4 * t42101 + 0.36021158228745895953e-3 * t36913 + 0.33105799917009430643e-4 * t36916 - 0.19211284388664477842e-2 * t36922 - 0.72042316457491791906e-3 * t36925 - 0.36021158228745895953e-3 * t36936 + t36943 + 0.10248087766267884742e-3 * t36948 - 0.42564599893297839398e-5 * t47855 + 0.12769379967989351819e-4 * t47857 + 0.12769379967989351819e-4 * t47861;
    (t47863,)
}
