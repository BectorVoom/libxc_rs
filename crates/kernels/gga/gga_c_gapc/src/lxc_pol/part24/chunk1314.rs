//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1314/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1314<F: Float>(t34638: F, t34641: F, t34644: F, t34647: F, t34651: F, t34654: F, t34658: F, t34661: F, t34663: F, t34666: F, t34669: F, t34673: F, t34676: F, t34679: F, t34682: F, t34686: F, t34689: F, t34692: F, t34695: F, t34698: F, t34701: F, t34704: F) -> (F, F) {
    let t38232 = -F::new(0.86880925264517213544e-4) * t34638 - F::new(0.14391604187239777452e-6) * t34641 + F::new(0.21587406280859666178e-5) * t34644 + F::new(0.19191204183684243232e-6) * t34647 + F::new(0.19666550313313802087e-7) * t34651 + F::new(0.98332751566569010433e-8) * t34654 - F::new(0.13111033542209201391e-7) * t34658 - F::new(0.43440462632258606772e-4) * t34661 - F::new(0.22745373045674261828e-5) * t34663 + F::new(0.10984838052999936404e-3) * t34666 + F::new(0.9275345110817126956e-4) * t34669;
    let t38244 = F::new(0.13900948042322754167e-2) * t34673 - F::new(0.18550690221634253912e-3) * t34676 + F::new(0.13900948042322754167e-2) * t34679 - F::new(0.70531721015421066817e-5) * t34682 - F::new(0.9275345110817126956e-4) * t34686 - F::new(0.43284943850479925794e-3) * t34689 + F::new(0.14659167650695868203e-4) * t34692 + F::new(0.17679409834076461864e-6) * t34695 - F::new(0.94854674673349911132e-9) * t34698 + F::new(0.13259557375557346398e-6) * t34701 - F::new(0.58999650939941406261e-7) * t34704;
    (t38232, t38244)
}
