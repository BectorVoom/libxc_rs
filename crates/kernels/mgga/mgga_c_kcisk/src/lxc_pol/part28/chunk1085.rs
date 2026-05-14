//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1085/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1085<F: Float>(t24810: F, t24826: F, t12325: F, t18766: F, t18768: F, t23915: F, t23920: F, t23930: F, t23933: F, t23936: F, t23939: F, t23942: F, t795: F, t9163: F, t18785: F, t18787: F, t23976: F, t23978: F, t23983: F, t23988: F, t23992: F, t23996: F, t23999: F, t24004: F, t24007: F) -> (F, F, F) {
    let t24827 = t24810 + t24826;
    let t24838 = t24827 * t795 - 0.30952962962962962962e-2 * t23915 + 0.25794135802469135802e-2 * t23920 + 0.74498e-1 * t12325 * t9163 - t18766 + t18768 - 0.38691203703703703703e-3 * t23930 + 0.34822083333333333332e-2 * t23933 + 0.92858888888888888886e-2 * t23936 + 0.92858888888888888886e-2 * t23939 - 0.61905925925925925924e-2 * t23942;
    let t24860 = 0.61905925925925925924e-2 * t23976 + t18785 + 0.15476481481481481481e-2 * t23978 - 0.10446625e-1 * t23983 + t18787 + 0.46429444444444444444e-2 * t23988 - 0.38691203703703703703e-2 * t23992 + 0.61905925925925925924e-2 * t23996 - 0.23214722222222222222e-2 * t23999 - 0.23214722222222222222e-2 * t24004 + 0.23214722222222222222e-2 * t24007;
    (t24827, t24838, t24860)
}
