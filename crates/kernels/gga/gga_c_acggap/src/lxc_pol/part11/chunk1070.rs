//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1070/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1070<F: Float>(t4773: F, t570: F, t30661: F, t30664: F, t30670: F, t30672: F, t30675: F, t30690: F, t30695: F, t30705: F, t30709: F, t34655: F, t34657: F, t34660: F, t34663: F, t34667: F, t34671: F, t34675: F, t34684: F) -> F {
    let t34686 = t570 * t4773;
    let t34688 = F::new(0.40015750243531754508e-2) * t30661 - t30664 - t30670 + t30672 - t34655 - F::new(0.17149607247227894789e-2) * t30675 - t34657 / F::new(96.0) + t34660 + F::new(0.31448092289604152068e-3) * t34663 + F::new(0.64311027177104605458e-3) * t34667 + F::new(0.47172138434406228102e-2) * t34671 + F::new(0.41930789719472202758e-3) * t34675 - F::new(0.34299214494455789578e-2) * t30690 + F::new(0.7145669686344956162e-3) * t30695 - F::new(0.10482697429868050689e-2) * t30705 - F::new(0.62896184579208304134e-3) * t30709 - F::new(0.64311027177104605458e-3) * t34684 - t34686 / F::new(48.0);
    t34688
}
