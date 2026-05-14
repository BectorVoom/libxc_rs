//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 824/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk824<F: Float>(t1798: F, t2741: F, t219: F, t5400: F, t7283: F, t639: F, t1416: F, t2683: F, t1815: F, t5521: F, t7803: F, t7805: F, t7806: F, t7808: F, t7810: F, t7812: F, t7833: F, t7837: F, t7841: F, t7843: F, t7846: F, t7848: F, t7850: F) -> (F, F, F, F) {
    let t7852 = 16.0 / 45.0 * t2741 * t1798;
    let t7853 = t5400 * t219;
    let t7854 = t7853 * t7283;
    let t7856 = 32.0 / 81.0 * t639 * t7854;
    let t7857 = t2683 * t1416;
    let t7858 = t1815 * t7857;
    let t7860 = 4.0 / 45.0 * t639 * t7858;
    let t7861 = -t7803 - t7805 - t7806 - t7808 - t7810 - t5521 - t7812 + t7833 + t7837 + t7841 + t7843 + t7846 - t7848 - t7850 + t7852 + t7856 - t7860;
    (t7852, t7856, t7860, t7861)
}
