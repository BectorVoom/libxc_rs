//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1335/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1335<F: Float>(t10972: F, t4614: F, t813: F, t29001: F, t14626: F, t3483: F, t10721: F, t1445: F, t28988: F, t28991: F, t29009: F, t29011: F, t29014: F, t29016: F, t29019: F, t29023: F, t29025: F, t29032: F, t29035: F, t32173: F, t807: F) -> F {
    let t33891 = F::cast_from(0.12269736305254639897e2_f64) * t813 * t4614 * t10972;
    let t33892 = F::cast_from(0.63904876589867916128e-1_f64) * t29001;
    let t33901 = F::cast_from(0.20449560508757733161e1_f64) * t813 * t14626 * t3483;
    let t33902 = -t33891 - t28988 + t28991 - t33892 + t29009 - t29011 - t29014 + t29016 - t29019 - t29023 + t29025 + t29032 - t29035 + F::cast_from(0.61348681526273199482e1_f64) * t807 * t4614 * t10721 + F::cast_from(0.23005755572352449806e1_f64) * t807 * t1445 * t32173 - t33901;
    t33902
}
