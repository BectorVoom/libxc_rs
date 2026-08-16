//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1273/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1273(t1992: f64, t550: f64, t6976: f64, t81028: f64, t22863: f64, t6979: f64, t12267: f64, t1336: f64, t22873: f64, t22877: f64, t3773: f64, t3777: f64, t3851: f64, t544: f64, t553: f64, t6988: f64, t6990: f64, t81011: f64, t81016: f64, t81019: f64, t81022: f64, t81031: f64, t81037: f64, t81039: f64, t81041: f64, t81043: f64, t81047: f64, t81050: f64, t81055: f64) -> f64 {
    let t81059 = t1992 * t6976 * t81028 * t550;
    let t81061 = t22863 * t6979;
    let t81063 = 3.0_f64 * t3773 * t6990 + t544 * t553 * t81011 + 0.49348022005446793095e-1_f64 * t81016 + 0.49348022005446793095e-1_f64 * t81019 - 0.24674011002723396548e-1_f64 * t81022 - 3.0_f64 * t1336 * t22873 * t3851 - 0.49348022005446793095e-1_f64 * t81031 - 3.0_f64 * t12267 * t6988 - 3.0_f64 * t3777 * t22877 - 0.57572692339687925277e-1_f64 * t81037 + 0.19190897446562641759e0_f64 * t81039 + 0.57572692339687925277e-1_f64 * t81041 - 0.34543615403812755166e0_f64 * t81043 - 0.78134368175290755733e-1_f64 * t81047 + 0.24674011002723396547e-1_f64 * t81050 + 0.14804406601634037928e0_f64 * t81055 - 0.82246703342411321825e-2_f64 * t81059 - 0.19190897446562641759e0_f64 * t81061;
    t81063
}
