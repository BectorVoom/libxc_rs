//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1257/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1257<F: Float>(t10961: F, t2197: F, t10713: F, t4614: F, t833: F, t24364: F, t955: F, t11001: F, t11004: F, t11113: F, t1445: F, t1710: F, t2043: F, t2087: F, t28022: F, t32748: F, t32753: F, t32756: F, t32759: F, t32761: F, t32764: F, t32766: F, t32769: F, t5666: F) -> F {
    let t32771 = F::cast_from(0.30674340763136599742e2_f64) * t2197 * t10961;
    let t32774 = F::cast_from(0.30674340763136599742e2_f64) * t833 * t4614 * t10713;
    let t32778 = F::cast_from(0.79445533226334281487e-1_f64) * t955 * t24364;
    let t32783 = -t32748 - t28022 + F::cast_from(0.51123901271894332905e0_f64) * t5666 * t11113 + t32753 - t32756 - t32759 + t32761 - t32764 - t32766 + t32769 + t32771 + t32774 + F::cast_from(0.35750489951850426669e0_f64) * t2043 * t11001 - t32778 - F::cast_from(0.69017266717057349418e1_f64) * t2087 * t1445 * t11004 * t1710;
    t32783
}
