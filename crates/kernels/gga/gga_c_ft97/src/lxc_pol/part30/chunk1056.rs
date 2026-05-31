//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1056/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1056<F: Float>(t1403: F, t27825: F, t2336: F, t35760: F, t1091: F, t141441: F, t141447: F, t141461: F, t141468: F, t1425: F, t1526: F, t193: F, t2320: F, t24181: F, t27469: F, t27860: F, t27863: F, t27866: F, t27869: F, t33540: F, t33547: F, t33552: F, t35757: F, t35761: F, t35766: F, t3704: F, t3746: F, t3837: F, t5996: F, t6002: F, t6062: F, t666: F, t6745: F, t6838: F, t684: F) -> F {
    let t151123 = t1403 * t27825;
    let t151126 = t1403 * t2336 * t35760;
    let t151139 = t141441 / F::cast_from(18.0_f64) - t1403 * t193 * t24181 * t3837 - t1403 * t3704 * t1425 * t3746 / F::cast_from(9.0_f64) - t141447 - t141461 / F::cast_from(36.0_f64) - t6002 * t27863 / F::cast_from(9.0_f64) - t6002 * t27866 / F::cast_from(9.0_f64) + t6002 * t27869 / F::cast_from(27.0_f64) - t6002 * t27860 / F::cast_from(9.0_f64) + t1403 * t666 * t6062 * t1091 / F::cast_from(18.0_f64) + t5996 * t35761 / F::cast_from(18.0_f64) + t6745 * t33547 / F::cast_from(18.0_f64) + t141468 / F::cast_from(18.0_f64) - t151123 / F::cast_from(9.0_f64) - t151126 / F::cast_from(54.0_f64) + t1403 * t666 * t6838 * t684 / F::cast_from(18.0_f64) - t33540 * t35766 / F::cast_from(6.0_f64) - t35757 * t33552 / F::cast_from(6.0_f64) - t1526 * t2320 * t27469 / F::cast_from(12.0_f64);
    t151139
}
