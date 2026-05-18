//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1062/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1062<F: Float>(t12580: F, t12584: F, t209: F, t3903: F, t575: F, t687: F, t1611: F, t3909: F, t1616: F, t11304: F, t11306: F, t11309: F, t11330: F, t11363: F, t11367: F, t12068: F, t12069: F, t12070: F, t12071: F, t12073: F, t12074: F, t12075: F, t12076: F, t12077: F, t12078: F, t12079: F, t12080: F, t12083: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12585 = t12580 + t12584;
    let t12586 = t12585 * t209;
    let t12587 = t3903 * t575;
    let t12588 = t12587 * t687;
    let t12589 = t1611 * t3909;
    let t12590 = t3909 * t687;
    let t12591 = t1616 * t12590;
    let t12592 = F::new(2.0) * t12591;
    let t12599 = F::new(0.18115908419564701086e-6) * t11304 - F::new(0.18115908419564701086e-6) * t11306 + F::new(0.6629778687778673199e-7) * t11309 + t12068 + t12069 - t12070 - t12071 + F::new(0.6629778687778673199e-7) * t11330 - t12073 - t12074 - t12075 + t12076 - t12077 - t12078 + t12079 - t12080 + F::new(0.78584976712469872988e-8) * t11363 - F::new(0.52838066223730378166e-7) * t11367 - t12083;
    (t12585, t12586, t12587, t12588, t12589, t12590, t12591, t12592, t12599)
}
