//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 857/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk857<F: Float>(t4791: F, t4794: F, t4798: F, t4975: F, t4979: F, t4981: F, t6961: F, t7865: F, t8559: F, t8560: F, t8592: F, t4806: F, t4992: F, t4996: F, t6002: F, t7870: F, t7874: F, t7876: F, t7878: F, t8634: F, t8636: F, t8638: F) -> (F, F) {
    let t9047 = t4975 - t8559 - t8560 + t4979 - t4981 - t6961 + F::new(0.571528e-1) * t7865 - t8592 - t4791 + t4794 + t4798;
    let t9051 = -t4806 + t8634 + t4992 - F::cast_from(0.675260332e-1_f64) * t6002 - t8636 - t8638 - F::cast_from(0.1350520664e0_f64) * t7870 - t7874 - t7876 + F::cast_from(0.2701041328e0_f64) * t7878 - t4996;
    (t9047, t9051)
}
