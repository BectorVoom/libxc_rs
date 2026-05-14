//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 734/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk734<F: Float>(t10850: F, t21181: F, t2917: F, t10838: F, t18877: F, t18900: F, t18902: F, t21839: F, t21843: F, t21847: F, t21852: F, t21856: F, t21863: F, t2265: F, t631: F, t21196: F, t4334: F) -> (F, F, F) {
    let t21867 = t2917 * t10850 * t21181;
    let t21870 = t631 * t21839 / 2.0 - 9.0 / 2.0 * t631 * t21843 + t631 * t21847 / 6.0 + 6.0 * t631 * t21852 + 2.0 / 27.0 * t631 * t21856 + t10838 + 4.0 / 3.0 * t18877 + 2.0 / 3.0 * t18900 - t18902 / 3.0 - t2265 * t21863 / 3.0 - t631 * t21867 / 3.0;
    let t21871 = t4334 * t21196;
    (t21867, t21870, t21871)
}
