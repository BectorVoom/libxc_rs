//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1086/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1086<F: Float>(t5879: F, t879: F, t425: F, t4889: F, t4911: F, t1435: F, t1449: F, t1452: F, t1384: F, t1433: F, t1434: F, t1445: F, t1450: F, t1453: F, t1498: F, t1505: F, t18786: F, t18839: F, t18843: F, t18904: F, t18924: F, t18927: F, t18941: F, t18973: F, t18995: F, t19013: F, t19033: F, t390: F, t437: F, t439: F, t453: F, t4754: F, t4759: F, t4994: F, t4998: F, t63: F, t76: F) -> (F, F, F, F, F) {
    let t19466 = t879 * t5879;
    let t19476 = t4889 * t425;
    let t19478 = t4911 * t425;
    let t19506 = t1435 * t1435;
    let t19510 = t1449 * t1449;
    let t19513 = t1452 * t1452;
    let t19518 = t18786 - 0.19263893255070628431e1 * t390 * t4998 + t18839 - t18843 + 0.41096e0 * t390 * t1433 * t1445 * t439 - 0.6609050294782684211e1 * t390 * t1450 * t1445 * t1453 * t437 + 0.1301229756036208781e0 * t390 * t4994 + t18941 - t18973 + 0.91082604192152556044e5 * t76 * t18924 * t18904 * t18927 - t18995 - 8.0 * t1434 * t4754 * t437 + 0.51947577317044391277e2 * t1505 * t19033 * t1384 - t19013 - 0.35089341735807877242e1 * t1498 * t19033 * t453 + 0.11579025239058625248e4 * t4759 * t19506 * t1453 + 0.19964560303604640732e6 * t63 / t19510 * t19506 / t19513;
    (t19466, t19476, t19478, t19506, t19518)
}
