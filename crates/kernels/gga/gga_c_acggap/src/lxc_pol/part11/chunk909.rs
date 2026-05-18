//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 909/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk909<F: Float>(t30889: F, t1988: F, t7792: F, t7796: F, t7799: F, t1078: F, t1980: F, t1982: F, t1983: F, t1997: F, t3036: F, t3213: F) -> (F, F, F, F, F) {
    let t30890 = F::new(0.12862205435420921092e-2) * t30889;
    let t30891 = t1988 * t7792;
    let t30893 = t7799 * t7796;
    let t30901 = t1980 * t1982 * t1078 * t1983;
    let t30904 = t3036 * t1997 * t3213;
    (t30890, t30891, t30893, t30901, t30904)
}
