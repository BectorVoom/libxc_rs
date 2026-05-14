//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 886/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk886<F: Float>(t299: F, t19904: F, t19939: F, t10947: F, t10948: F, t10949: F, t10950: F, t13: F, t16584: F, t17681: F, t18793: F, t4640: F, t4905: F, t5197: F, t5490: F) -> (F,) {
    let t300 = 10000000.0 <= t299;
    let t19941 = piecewise3(t300, 0.0, t19904 + t19939);
    let tv3rho32 = t10947 + t10948 + t10949 + t10950 + t4640 + t4905 + t5197 + t5490 + t13 * (t16584 + t17681 + t18793 + t19941);
    (tv3rho32,)
}
