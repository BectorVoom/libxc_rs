//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 229/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk229<F: Float>(t772: F, t876: F, t284: F, t316: F, t344: F, t366: F, t724: F, t727: F, t730: F, t731: F, t763: F, t771: F, t788: F, t794: F, t795: F, t799: F, t802: F, t821: F, t828: F, t832: F, t835: F) -> (F, F) {
    let t877 = t772 * t876;
    let t880 = t344 + t366 + t724 - t727 - t730 - F::new(0.46971924784082831588e-3) * t731 * t316 + F::new(0.28183154870449698953e-3) * t763 * t316 - F::new(0.28183154870449698953e-3) * t771 * t788 - F::new(0.93943849568165663176e-5) * t794 * t795 + F::new(0.16703216453219854913e-4) * t799 * t802 + F::new(0.28183154870449698953e-3) * t284 * t821 + F::new(0.1370014472869082588e-4) * t828 * t832 - F::new(0.28183154870449698953e-3) * t835 * t877;
    (t877, t880)
}
