//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1331/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1331<F: Float>(t34680: F, t26673: F, t544: F, t9287: F, t26629: F, t3394: F, t4130: F, t20535: F, t6578: F, t12881: F, t4382: F, t874: F) -> (F, F, F, F, F) {
    let t34681 = F::new(0.29792074959875355558e-1) * t34680;
    let t34683 = t544 * t26673 * t9287;
    let t34684 = F::new(0.14896037479937677779e-1) * t34683;
    let t34686 = t544 * t26629 * t9287;
    let t34687 = F::new(0.29792074959875355558e-1) * t34686;
    let t34688 = t4130 * t3394;
    let t34690 = t20535 * t34688 * t6578;
    let t34691 = F::new(0.11502877786176224903e1) * t34690;
    let t34699 = F::new(0.53625734927775640005e1) * t544 * t4382 * t874 * t12881;
    (t34681, t34684, t34687, t34691, t34699)
}
