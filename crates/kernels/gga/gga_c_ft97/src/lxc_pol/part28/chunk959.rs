//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 959/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk959<F: Float>(t35006: F, t92: F, t138411: F, t138445: F, t138706: F, t139563: F, t1969: F, t23413: F, t24080: F, t26801: F, t26815: F, t26817: F, t26822: F, t26950: F, t27416: F, t32714: F, t32717: F, t32724: F, t3450: F, t34975: F, t40830: F, t5772: F, t5773: F, t5775: F, t6584: F, t925: F, t9432: F) -> (F,) {
    let t147073 = t35006 * t92;
    let t147091 = -t5772 * t138445 * t27416 / 3.0 + 2.0 / 9.0 * t5772 * t24080 * t26822 - t138411 / 27.0 - t5772 * t1969 * t139563 * t925 / 9.0 - t26817 * t32724 / 18.0 - t147073 * t5775 / 18.0 + 2.0 * t5772 * t9432 * t5773 * t26950 + t32714 * t26801 / 9.0 - t138706 * t6584 / 18.0 - 4.0 * t5772 * t40830 * t32717 * t3450 + t32714 * t26815 - t23413 * t34975 / 18.0;
    (t147091,)
}
