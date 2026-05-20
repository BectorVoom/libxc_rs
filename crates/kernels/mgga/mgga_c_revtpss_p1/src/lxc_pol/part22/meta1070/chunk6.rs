//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3833/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3833<F: Float>(t1424: F, t14268: F, t1903: F, t22390: F, t4076: F, t4077: F, t4078: F, t46362: F, t47504: F, t47510: F, t47512: F, t47516: F, t47521: F, t47525: F, t47527: F, t47534: F, t47873: F, t47876: F, t47885: F, t47893: F, t47899: F, t6895: F, t73666: F, t73671: F, t73673: F, t73676: F) -> F {
    let t73700 = -F::cast_from(0.26019841438354088049e-1_f64) * t73666 + F::cast_from(0.65854491829355115984e-1_f64) * t73671 + t47504 - F::cast_from(0.73171657588172351096e-2_f64) * t73673 - F::cast_from(0.43902994552903410656e-1_f64) * t73676 + F::cast_from(0.11565819519348392139e-2_f64) * t47510 + F::cast_from(0.13170898365871023197e1_f64) * t22390 * t4078 + F::cast_from(0.10975748638225852664e-1_f64) * t47873 + F::cast_from(0.19514881078765566038e-1_f64) * t47876 - F::cast_from(0.22089088168956307394e-3_f64) * t47512 + F::cast_from(0.15805078039045227836e2_f64) * t1424 * t46362 * t6895 * t4077 - F::cast_from(0.46263278077393568556e-2_f64) * t47885 - F::cast_from(0.13009920719177044025e-1_f64) * t47516 - F::cast_from(0.23131639038696784278e-2_f64) * t47521 + F::cast_from(0.13009920719177044025e-1_f64) * t47525 + F::cast_from(0.26341796731742046394e1_f64) * t1424 * t4076 * t1903 * t14268 - F::cast_from(0.39029762157531132076e-1_f64) * t47893 + F::cast_from(0.2601984143835408805e-1_f64) * t47527 - F::cast_from(0.520396828767081761e-2_f64) * t47899 + F::cast_from(0.65049603595885220126e-3_f64) * t47534;
    t73700
}
