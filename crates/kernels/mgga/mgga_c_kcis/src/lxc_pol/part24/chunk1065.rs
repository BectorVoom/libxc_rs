//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1065/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1065<F: Float>(t27835: F, t27889: F, t27939: F, t27983: F, t393: F, t1141: F, t8060: F, t1203: F, t1820: F, t26868: F, t26871: F, t5039: F) -> (F, F, F, F, F, F) {
    let t27985 = t27835 + t27889 + t27939 + t27983;
    let t27986 = t27985 * t393;
    let t27987 = t8060 * t1141;
    let t27988 = t27987 * t1203;
    let t27989 = t26868 * t1820;
    let t27991 = F::new(2.0) * t26871 * t5039;
    (t27985, t27986, t27987, t27988, t27989, t27991)
}
