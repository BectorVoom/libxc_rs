//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1050/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1050<F: Float>(t23609: F, t3347: F, t10132: F, t6313: F, t3355: F, t10249: F, t6305: F, t2268: F, t31585: F, t426: F, t535: F, t1222: F, t3344: F, t10262: F, t484: F, t1217: F, t3351: F) -> (F, F, F, F, F, F, F, F) {
    let t31672 = 0.37940008847568199467e-1 * t23609 * t3347;
    let t31674 = 0.2276400530854091968e0 * t6313 * t10132;
    let t31679 = 0.25293339231712132977e-1 * t23609 * t3355;
    let t31681 = 0.68292015925622759036e0 * t6305 * t10249;
    let t31685 = 0.56910013271352299198e-1 * t2268 * t535 * t31585 * t426;
    let t31687 = t1222 * t3344;
    let t31688 = 0.31616674039640166222e-2 * t31687;
    let t31689 = t484 * t10262;
    let t31690 = 0.31616674039640166222e-2 * t31689;
    let t31691 = t1217 * t3351;
    (t31672, t31674, t31679, t31681, t31685, t31688, t31690, t31691)
}
