//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1184/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1184<F: Float>(t30793: F, t30798: F, t30800: F, t30804: F, t30809: F, t30812: F, t30814: F, t30818: F, t30821: F, t30830: F, t30840: F, t30844: F, t30846: F, t30848: F, t30852: F, t30854: F, t35001: F) -> F {
    let t37344 = -t35001 / F::cast_from(48.0_f64) - F::cast_from(0.75475421495049964964e-2_f64) * t30793 - F::cast_from(0.42874018118069736972e-3_f64) * t30798 + F::cast_from(0.34299214494455789578e-2_f64) * t30800 + F::cast_from(0.37737710747524982483e-2_f64) * t30804 + F::cast_from(0.94344276868812456204e-2_f64) * t30809 + F::cast_from(0.13719685797782315831e-1_f64) * t30812 - F::cast_from(0.13719685797782315831e-1_f64) * t30814 + F::cast_from(0.37737710747524982482e-2_f64) * t30818 + F::cast_from(0.28582678745379824648e-2_f64) * t30821 - F::cast_from(0.41930789719472202758e-2_f64) * t30830 - F::cast_from(0.25724410870841842184e-2_f64) * t30840 - F::cast_from(0.16006300097412701803e-1_f64) * t30844 + F::cast_from(0.62896184579208304137e-2_f64) * t30846 - F::cast_from(0.94344276868812456206e-2_f64) * t30848 - F::cast_from(0.15724046144802076034e-2_f64) * t30852 + F::cast_from(0.51448821741683684368e-2_f64) * t30854;
    t37344
}
