//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1091/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1091(t36913: f64, t36916: f64, t36922: f64, t36925: f64, t36936: f64, t36943: f64, t36948: f64, t42087: f64, t42101: f64, t47831: f64, t47833: f64, t47835: f64, t47840: f64, t47845: f64, t47855: f64, t47857: f64, t47861: f64) -> f64 {
    let t47863 = -0.5987120850931904282e-1_f64 * t47831 - 0.71845450211182851384e0_f64 * t47833 + 0.15965655602485078085e0_f64 * t47835 + t42087 - 0.31923449919973379548e-4_f64 * t47840 + 0.95770349759920138644e-4_f64 * t47845 - 0.59590439850616975158e-4_f64 * t42101 + 0.36021158228745895953e-3_f64 * t36913 + 0.33105799917009430643e-4_f64 * t36916 - 0.19211284388664477842e-2_f64 * t36922 - 0.72042316457491791906e-3_f64 * t36925 - 0.36021158228745895953e-3_f64 * t36936 + t36943 + 0.10248087766267884742e-3_f64 * t36948 - 0.42564599893297839398e-5_f64 * t47855 + 0.12769379967989351819e-4_f64 * t47857 + 0.12769379967989351819e-4_f64 * t47861;
    t47863
}
