//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1030/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1030<F: Float>(t150664: F, t3773: F, t1701: F, t27724: F, t108897: F, t123543: F, t13443: F, t150655: F, t150659: F, t150662: F, t226: F, t237: F, t25: F, t27711: F, t27713: F, t30671: F, t33362: F, t35462: F, t36801: F, t3723: F, t3725: F, t3762: F, t3777: F, t3782: F, t5009: F, t677: F, t7446: F) -> F {
    let t150665 = t150664 * t3773;
    let t150668 = t1701 * t27724;
    let t150684 = F::new(0.88910709717637694816e-2) * t123543 * t33362 + F::new(0.13519760450715832853e-3) * t3723 * t7446 * t226 * t3725 - F::new(0.11854761295685025975e-1) * t30671 * t150655 - F::new(0.90822088511484663583e-3) * t150659 - F::new(0.11738898233082762229e-1) * t150662 - F::new(0.13784064983740990796e-3) * t150665 * t3777 + F::new(0.44455354858818847408e-2) * t108897 * t150668 + F::new(0.44455354858818847408e-2) * t27711 * t1701 * t27713 - F::new(0.44455354858818847408e-2) * t13443 * t150668 - F::new(0.16779431174156321371e-9) * t677 * t237 * t5009 * t35462 * t25 * t3762 - F::new(0.11854761295685025975e-1) * t36801 * t3782;
    t150684
}
