//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 821/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk821<F: Float>(t1092: F, t2555: F, t191: F, t2786: F, t3304: F, t9556: F, t9558: F, t9561: F, t9565: F, t9568: F, t9570: F, t9572: F, t9579: F, t9581: F, t9584: F, t9587: F) -> F {
    let t9589 = t1092 * t2555;
    let t9591 = t2786 * t191;
    let t9592 = t9591 * t3304;
    let t9594 = -F::new(0.49196596498842592595e-6) * t9556 - F::new(0.16908181191593721013e-4) * t9558 + F::new(0.72463633678258804342e-6) * t9561 + F::new(0.84410248952307505288e-7) * t9565 + F::new(0.16882049790461501058e-6) * t9568 - F::new(0.30660168560756614104e-3) * t9570 + F::new(0.4637672555408563478e-4) * t9572 - F::new(0.84410248952307505288e-7) * t9579 - F::new(0.98393192997685185188e-5) * t9581 + F::new(0.38010404803226280926e-3) * t9584 + F::new(0.14492726735651760868e-5) * t9587 + F::new(0.33816362383187442026e-4) * t9589 - F::new(0.14492726735651760868e-5) * t9592;
    t9594
}
