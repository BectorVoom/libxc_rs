//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1263/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1263<F: Float>(t125782: F, t122346: F, t122435: F, t122438: F, t125780: F, t125785: F, t125793: F, t125797: F, t125799: F, t125803: F, t14224: F, t27846: F, t32690: F, t32719: F) -> F {
    let t128770 = F::new(0.263521689745817692e-2) * t125782;
    let t128781 = F::new(0.225875734067843736e-2) * t125780 - t122435 + t128770 + F::new(0.8673628188205199462e0) * t32690 * t27846 + F::new(0.7437465841810202164e-3) * t125785 - t122438 - F::new(0.56468933516960933999e-3) * t125793 + F::new(0.37645955677973955999e-4) * t125797 - F::new(0.66934509195437693771e-4) * t125799 - F::new(0.11423947533020470523e1) * t32719 * t122346 * t14224 + F::new(0.112937867033921868e-1) * t125803;
    t128781
}
