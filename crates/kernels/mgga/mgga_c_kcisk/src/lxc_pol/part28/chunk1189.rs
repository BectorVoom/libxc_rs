//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1189/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1189<F: Float>(t2785: F, t32942: F, t32990: F, t33002: F, t33052: F, t34078: F, t34261: F, t34264: F, t34267: F, t34270: F, t34275: F, t34278: F, t34280: F, t9649: F, t9664: F, t9940: F) -> (F,) {
    let t34283 = -0.120625e-1 * t9649 * t34078 - 0.23280625000000000001e-2 * t33002 * t34078 + 0.10416666666666666667e-1 * t32942 * t9940 + 0.10416666666666666667e-1 * t32990 * t9940 + 0.10416666666666666667e-1 * t9664 * t34261 - 0.10416666666666666667e-1 * t34264 * t2785 - 0.10416666666666666667e-1 * t34267 * t2785 + 0.27777777777777777779e-1 * t34270 * t2785 + 0.27777777777777777779e-1 * t34275 * t2785 + 0.16581944444444444444e-2 * t34278 - 0.44218518518518518517e-2 * t34280 + 0.13402777777777777778e-2 * t33052;
    (t34283,)
}
