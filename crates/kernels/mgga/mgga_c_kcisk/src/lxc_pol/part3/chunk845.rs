//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 845/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk845<F: Float>(t1192: F, t12900: F, t3634: F, t3672: F, t1167: F, t3676: F, t3680: F, t317: F, t3675: F, t305: F, t1190: F, t3640: F) -> (F, F, F, F, F) {
    let t12902 = F::new(3.0) * t12900 * t1192;
    let t12904 = F::new(3.0) * t3634 * t3672;
    let t12905 = t1167 * t3676;
    let t12907 = F::cast_from(0.48245472966453314466e2_f64) * t12905 * t3680;
    let t12909 = F::new(1.0) / t3675 / t317;
    let t12910 = t305 * t12909;
    let t12911 = t3640 * t1190;
    (t12902, t12904, t12907, t12910, t12911)
}
