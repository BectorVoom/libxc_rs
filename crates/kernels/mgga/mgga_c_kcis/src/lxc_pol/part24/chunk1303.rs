//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1303/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1303<F: Float>(t15573: F, t28947: F, t2173: F, t100704: F, t100707: F, t100736: F, t100741: F, t100749: F, t100762: F, t1250: F, t2175: F, t70994: F, t93606: F, t96264: F, t96270: F) -> (F, F) {
    let t101501 = t15573 * t28947;
    let t101502 = t2173 * t101501;
    let t101509 = -F::new(0.49745833333333333332e-2) * t100704 - F::new(0.55273148148148148147e-2) * t100707 - F::new(0.22109259259259259258e-2) * t100736 - F::new(0.7369753086419753086e-3) * t100741 + t96264 + F::new(0.15445601851851851852e-3) * t93606 + F::new(0.16581944444444444444e-2) * t100749 + F::new(0.23168402777777777778e-3) * t101502 - F::new(0.69505208333333333333e-3) * t70994 * t1250 * t2175 - F::new(0.88437037037037037035e-2) * t96270 + F::new(0.66327777777777777776e-2) * t100762;
    (t101501, t101509)
}
