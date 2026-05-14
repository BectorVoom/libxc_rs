//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1133/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1133<F: Float>(t33689: F, t10811: F, t28837: F, t2021: F, t7372: F, t8520: F, t10930: F, t10931: F, t32948: F, t2615: F, t326: F, t33155: F, t10847: F, t16036: F, t7630: F, t33557: F) -> (F, F, F, F, F, F, F) {
    let t33690 = 0.38342925953920749676e0 * t33689;
    let t33691 = t10811 * t28837;
    let t33692 = 0.17875244975925213335e0 * t33691;
    let t33694 = t2021 * t8520 * t7372;
    let t33695 = 0.29792074959875355558e-1 * t33694;
    let t33702 = 0.55213813373645879534e2 * t10930 * t10931 * t32948;
    let t33705 = 0.46011511144704899612e1 * t2615 * t326 * t33155;
    let t33708 = 0.95334639871601137784e0 * t7630 * t16036 * t10847;
    let t33711 = 0.92023022289409799224e1 * t2615 * t326 * t33557;
    (t33690, t33692, t33695, t33702, t33705, t33708, t33711)
}
