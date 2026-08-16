//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1703/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1703<F: Float>(t1379: F, t9709: F, t2689: F, t3952: F, t1413: F, t3889: F, t547: F, t807: F, t9646: F, t2236: F, t66: F) -> (F, F, F, F, F, F, F) {
    let t9711 = F::cast_from(0.12846167376791569079e-2_f64) * t1379 * t9709;
    let t9712 = t2689 * t3952;
    let t9714 = t1413 * t3889;
    let t9715 = t547 * t9714;
    let t9716 = t807 * t9715;
    let t9718 = t9646 * t547;
    let t9720 = F::cast_from(1.0_f64) / t66 / t2236;
    (t9711, t9712, t9714, t9715, t9716, t9718, t9720)
}
