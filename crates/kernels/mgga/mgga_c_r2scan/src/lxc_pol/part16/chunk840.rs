//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 840/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk840<F: Float>(t170: F, t8892: F, t596: F, t7647: F, t7650: F, t7653: F, t7656: F, t7659: F, t7661: F, t7662: F, t7664: F, t7667: F, t7669: F, t7671: F) -> F {
    let t8893 = t8892 * t170;
    let t8896 = F::new(0.38527786510141256861e1) * t7647 + F::new(0.3429168e0) * t7650 + t7653 + t7656 + t7659 + t7661 - F::new(0.2077903092681775651e3) * t7662 + F::new(0.70178683471615754484e1) * t7664 - F::new(0.67745118933333333332e-2) * t7667 + F::new(0.14458108400402319789e-1) * t7669 - F::new(40.0) * t7671 - F::new(0.675260332e-1) * t596 * t8893;
    t8896
}
